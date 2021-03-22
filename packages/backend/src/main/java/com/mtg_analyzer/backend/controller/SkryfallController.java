package com.mtg_analyzer.backend.controller;

import com.mtg_analyzer.backend.exception.BadRequestException;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;

@RestController
@RequestMapping("/skryfall")
public class SkryfallController {
    private final String baseUrl = "https://api.scryfall.com";

    private final RestTemplate restTemplate = getRestTemplate();

    @Autowired
    public SkryfallController() {}

    public String getBaseUrl() {
        return this.baseUrl;
    }

    // PLEASE REVIEW SKRYFALL'S DOCUMENTATION: https://scryfall.com/docs/api

    /**
     * Returns a JSON string representing the cards that match the query param passed in
     * @param q A fulltext search query, it needs to be properly encoded
     *
     * @example https://api.scryfall.com/cards/search?q=krenko
     * @example https://api.scryfall.com/cards/search?q=krenko,%20mob
     */
    @GetMapping("/cards/search")
    public ResponseEntity<String> cardsSearch(@RequestParam() String q) {

        return this.restTemplate.getForEntity(
                this.baseUrl + "/cards/search?q=" + q,
                String.class
        );
    }

    /**
     * Returns a JSON string representing the cards that have the exact name (when exact param is used) or match
     * a fuzzy keyword search (when the fuzzy param is used)
     * @param fuzzy Fuzzy text to search cards with, it needs to be properly encoded
     * @param exact Exact name of card to search for, it needs to be properly encoded
     *
     * @example http://localhost:8080/skryfall/cards/named?fuzzy=krenko,%20mob
     */
    @GetMapping("/cards/named")
    public ResponseEntity<String> cardsNamed(@RequestParam(required = false) String fuzzy, @RequestParam(required = false) String exact) {

        if (fuzzy == null && exact == null) {
            throw new BadRequestException("You must select either fuzzy or exact search.");
        }

        String q = fuzzy != null ? "fuzzy=" + fuzzy : "exact=" + exact;

        return this.restTemplate.getForEntity(
                this.baseUrl + "/cards/named?" + q,
                String.class
        );

    }

    /**
     * Returns a JSON string representing the cards that had the id in the request
     * @param id refers to the Skryfall ID, represented with a UUID in their database
     * @example https://api.scryfall.com/cards/e9d5aee0-5963-41db-a22b-cfea40a967a3
     */
    @GetMapping("/cards")
    public ResponseEntity<String> cardsWithId(@RequestParam() String id) {

        return this.restTemplate.getForEntity(
                this.baseUrl + "/cards/" + id,
                String.class
        );
    }

    @Bean
    public RestTemplate getRestTemplate() {
        return new RestTemplate();
    }
}