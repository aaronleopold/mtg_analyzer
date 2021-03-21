package com.mtg_analyzer.backend;

import com.mtg_analyzer.backend.controllers.SkryfallController;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class BackendApplicationTests {

	@Autowired
	private SkryfallController skryfallController;


	@Test
	void contextLoads() {
		assertThat(skryfallController).isNotNull();
	}

	// TODO: figure out testing for this
	@Test
	public void getCardsNamedFuzzyKrenkoShouldFail() throws Exception {
//		ResponseEntity<String> response = this.restTemplate.getForEntity(
//				"http://localhost:8080/skryfall/cards/named?fuzzy=krenko",
//				String.class
//		);
//
//		assertThat(response.getStatusCode()).isEqualTo(HttpStatus.valueOf(500));

	}

}
