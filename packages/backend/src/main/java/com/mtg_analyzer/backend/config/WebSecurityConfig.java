package com.mtg_analyzer.backend.config;

//// TODO: big todo, this disables security. I don't want that, this is just for my own ease of testing.
//// TODO: https://www.innoq.com/en/blog/cookie-based-spring-security-session/ <--> read me

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.autoconfigure.security.servlet.PathRequest;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configuration.WebSecurityConfigurerAdapter;
import org.springframework.security.crypto.password.PasswordEncoder;


@Configuration
@EnableWebSecurity
public class WebSecurityConfig extends WebSecurityConfigurerAdapter {

    @Override
    protected void configure(HttpSecurity http) throws Exception {
        http
            .csrf().disable()
            .authorizeRequests()
            .antMatchers(
                    "/user/**", "skryfall/**"
            )
            .permitAll()
            .anyRequest()
            .authenticated();
    }

}