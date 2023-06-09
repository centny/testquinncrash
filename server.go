package main

import (
	"context"
	"crypto/rand"
	"crypto/tls"
	"fmt"
	"time"

	"github.com/quic-go/quic-go"
)

func listenConfig(certFile, keyFile string) (config *tls.Config, err error) {
	var cert tls.Certificate
	cert, err = tls.LoadX509KeyPair(certFile, keyFile)
	if err != nil {
		return
	}
	config = &tls.Config{InsecureSkipVerify: false}
	config.Certificates = append(config.Certificates, cert)
	config.Rand = rand.Reader
	return
}

func quicListen(addr string, tls *tls.Config) (ln *quic.Listener, err error) {
	quicConf := &quic.Config{EnableDatagrams: true}
	quicConf.KeepAlivePeriod = time.Second
	ln, err = quic.ListenAddr(addr, tls, quicConf)
	return
}

func main() {
	config, err := listenConfig("certs/server.crt", "certs/server.key")
	if err != nil {
		panic(err)
	}
	config.NextProtos = []string{"test"}
	ln, err := quicListen(":3100", config)
	if err != nil {
		panic(err)
	}
	fmt.Printf("listen on %v\n", ln.Addr())
	for {
		conn, err := ln.Accept(context.Background())
		if err != nil {
			fmt.Printf("accept conn fail %v\n", err)
			continue
		}
		fmt.Printf("accept from %v\n", conn.RemoteAddr())
		stream, err := conn.AcceptStream(conn.Context())
		if err != nil {
			fmt.Printf("accept stream fail %v\n", err)
			continue
		}
		fmt.Printf("bixx from %v\n", conn.RemoteAddr())
		go func(name string, s quic.Stream) {
			buf := make([]byte, 1024)
			for {
				_, err := s.Read(buf)
				if err != nil {
					break
				}
			}
			fmt.Printf("closed from %v\n", name)
			s.Close()
		}(conn.RemoteAddr().String(), stream)
		go func(name string, s quic.Stream) {
			for {
				fmt.Printf("to send-->\n")
				n, err := fmt.Fprintf(s, "testing\n")
				if err != nil {
					break
				}
				fmt.Printf("send-->%v\n", n)
				time.Sleep(time.Second)
			}
			fmt.Printf("closed from %v\n", name)
			s.Close()
		}(conn.RemoteAddr().String(), stream)
	}
}
