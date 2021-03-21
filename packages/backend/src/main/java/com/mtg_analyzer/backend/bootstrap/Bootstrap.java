package com.mtg_analyzer.backend.bootstrap;

import com.mtg_analyzer.backend.entities.User;
import com.mtg_analyzer.backend.repositories.UserRepository;
import org.springframework.boot.CommandLineRunner;
import org.springframework.stereotype.Component;

@Component
public class Bootstrap implements CommandLineRunner {

    private final UserRepository userRepository;

    public Bootstrap(UserRepository userRepository) {
         this.userRepository = userRepository;
     }

    @Override
    public void run(String... args) throws Exception {
        System.out.println("\n=============================================");
        System.out.println("STARTING BOOTSTRAP...\n");

        User aaron = new User(
                "oromei",
                "aaron@aaronbleopold.com",
                "Aaron",
                "Leopold",
                "mysupersecurepassword"
        );

        this.userRepository.saveAndFlush(aaron);

        System.out.println("Created (super cool) user " + aaron.getUsername() + ".");

        System.out.println("=============================================\n");

    }
}
