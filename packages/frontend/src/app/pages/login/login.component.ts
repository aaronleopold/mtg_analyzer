import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/app/services/auth-service/auth.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: [],
})
export class LoginComponent implements OnInit {
  constructor(private auth: AuthService) {}

  ngOnInit(): void {}

  // TODO: type this
  attemptLogin(e: any) {
    const { username, password } = e;

    this.auth.login(username, password).subscribe(
      (res) => console.log('RESULT:', res),
      (err) => console.log('ERROR:', err)
    );
  }
}
