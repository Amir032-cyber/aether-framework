package main

import (
	"context"
	"log"
	"net"

	"google.golang.org/grpc"
	// TODO: Implémenter la génération des stubs Go à partir des fichiers proto
)

type server struct {
	// aether.UnimplementedSecurityModuleServer
}

func main() {
	lis, err := net.Listen("tcp", ":50052")
	if err != nil {
		log.Fatalf("Échec de l'écoute : %v", err)
	}
	s := grpc.NewServer()
	// aether.RegisterSecurityModuleServer(s, &server{})
	log.Printf("Module NeuroSploit à l'écoute sur %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Échec du service : %v", err)
	}
}
