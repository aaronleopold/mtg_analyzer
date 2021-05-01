import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';
import { FormsModule } from '@angular/forms';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { HomeComponent } from './pages/home/home.component';
import { LoginComponent } from './pages/login/login.component';
import { NavigationComponent } from './components/navigation/navigation.component';
import { FooterComponent } from './components/footer/footer.component';
import { DeckDetailComponent } from './pages/deck-detail/deck-detail.component';
import { DeckSearchComponent } from './pages/deck-search/deck-search.component';
import { BannerComponent } from './components/banner/banner.component';
import { ButtonComponent } from './components/ui/button/button.component';
import { RadioComponent } from './components/ui/radio/radio.component';

@NgModule({
  declarations: [
    AppComponent,
    HomeComponent,
    LoginComponent,
    NavigationComponent,
    FooterComponent,
    DeckDetailComponent,
    DeckSearchComponent,
    BannerComponent,
    ButtonComponent,
    RadioComponent,
  ],
  imports: [BrowserModule, AppRoutingModule, HttpClientModule, FormsModule],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
