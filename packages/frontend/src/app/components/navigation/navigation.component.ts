import { Component, OnInit } from '@angular/core';
import { Location } from '@angular/common';
import { Router } from '@angular/router';

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.css'],
})
export class NavigationComponent implements OnInit {
  public route: string = '';

  constructor(location: Location, router: Router) {
    router.events.subscribe((val) => {
      this.route = location.path();
    });
  }

  ngOnInit(): void {}
}
