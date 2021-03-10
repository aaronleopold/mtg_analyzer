package com.mtg_analyzer.backend.bootstrap;

import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;

@Component
public class Bootstrap implements CommandLineRunner {

    // private final CardRepository cardRepository;

    // public Bootstrap(CardRepository cardRepository) {
    //     this.cardRepository = cardRepository;
    // }

    public Bootstrap() {}


    @Override
    public void run(String... args) throws Exception {
        System.out.println("\n=============================================");
        System.out.println("STARTING BOOTSTRAP...\n");
        System.out.println("This doesn't do anything yet!");
        System.out.println("=============================================\n");

    }
}
