package com.mtg_analyzer.backend.service;

import com.mtg_analyzer.backend.entity.User;
import com.mtg_analyzer.backend.exception.AuthenticationException;
import com.mtg_analyzer.backend.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

@Service
public class UserService implements UserDetailsService {

    private final UserRepository userRepository;

    @Autowired
    public UserService(UserRepository userRepository) {
        this.userRepository = userRepository;
    }

    @Override
    public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {

        // load user from repository
        return this.userRepository
                .findByUsername(username)
                .orElseThrow(
                        () -> new UsernameNotFoundException("TODO: make me")
                );

    }

    public UserDetails registerNewUser(
            String username, String email, String firstName, String lastName, String password
    ) {

        try {
            loadUserByUsername(username);
            // I don't want to tell people if the user exists
            throw new AuthenticationException("Something went wrong during the registration. Please try again.");
        } catch (UsernameNotFoundException e) {
            // I WANT this to be caught, it means no user with the username

            // FIXME: wrap in another try??
            User newUser = new User(
              username,
              email,
              firstName,
              lastName,
              password
            );

            userRepository.saveAndFlush(newUser);

            return loadUserByUsername(username);
        }

    }
}
