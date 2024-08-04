import { Component } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatTabsModule } from '@angular/material/tabs';
import { ReactiveFormsModule } from '@angular/forms';

import { UserDetailsComponent } from '../user-details/user-details.component';
import { UserFlightDetailsComponent } from '../user-flight-details/user-flight-details.component';
import { sampleAnimeHeroesImageUrls } from 'src/assets/sample.animeHeroesImagesUrls';

import { ToolbarComponent } from 'src/app/shared/toolbar/toolbar.component';

@Component({
  selector: 'app-user-profile',
  standalone: true,
  imports: [
    UserDetailsComponent,
    UserFlightDetailsComponent,
    ToolbarComponent,
    MatButtonModule,
    ReactiveFormsModule,
    MatCardModule,
    MatFormFieldModule,
    MatInputModule,
    MatTabsModule
  ],
  templateUrl: './user-profile.component.html',
  styleUrl: './user-profile.component.scss'
})
export class UserProfileComponent {
  user = {
    name: 'John Doe',
    email: 'johndoe@example.com',
    status: 'BRONZE',
    imageUrl: sampleAnimeHeroesImageUrls[Math.floor(Math.random() * sampleAnimeHeroesImageUrls.length)]
  };

  constructor() {

  }

  onSubmit() {

  }

  onCancel() {

  }
}
