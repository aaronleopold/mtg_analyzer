package com.mtg_analyzer.backend.entity;

import javax.persistence.*;
import java.io.Serializable;

@Entity
public class Deck implements Serializable {

    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    Long id;

    @Column(nullable = false)
    private String name;

    @OneToOne
    User owner;

    // TODO: cards ?? not sure how to represent them since calling external api

    // FIXME: should I nest this enum?
    public enum DECK_VISIBILITY {
        PRIVATE,
        PUBLIC,
        UNLISTED
    }

}
