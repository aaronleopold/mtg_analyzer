package com.mtg_analyzer.backend.config;

import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.EnableWebMvc;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurerAdapter;

// FIXME: https://www.baeldung.com/spring-cors --> latest I could find but it says in IDE
// that WebMvcConfigurerAdapter is deprecated...
// FIXME: I want to restrict this a bit
@Configuration
@EnableWebMvc
public class CorsConfiguration extends WebMvcConfigurerAdapter {
    @Override
    public void addCorsMappings(CorsRegistry registry) {
        registry.addMapping("/**");
    }
}