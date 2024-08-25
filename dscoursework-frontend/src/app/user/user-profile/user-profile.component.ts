import { Component, signal, Signal } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatTabsModule } from '@angular/material/tabs';
import { ReactiveFormsModule } from '@angular/forms';

import { UserDetailsComponent } from '../user-details/user-details.component';
import { TicketsComponent } from '../tickets/tickets.component';
import { BalanceDetailsComponent } from '../balance-details/balance-details.component';

import { sampleAnimeHeroesImageUrls } from 'src/assets/sample.animeHeroesImagesUrls';
import { ToolbarComponent } from 'src/app/shared/toolbar/toolbar.component';
import { BalanceHistory, TicketResponse } from 'src/app/services';
import { BalanceHistoryComponent } from '../balancy-history/balance-history.component';

@Component({
  selector: 'app-user-profile',
  standalone: true,
  imports: [
    MatButtonModule,
    ReactiveFormsModule,
    MatCardModule,
    MatFormFieldModule,
    MatInputModule,
    MatTabsModule,

    ToolbarComponent,
    TicketsComponent,
    UserDetailsComponent,
    BalanceHistoryComponent,
  ],
  templateUrl: './user-profile.component.html',
  styleUrl: './user-profile.component.scss'
})
export class UserProfileComponent {
  user = {
    name: 'Jim Keller',
    email: 'johndoe@example.com',
    status: 'BRONZE',
    imageUrl: sampleAnimeHeroesImageUrls[
      Math.floor(Math.random() * sampleAnimeHeroesImageUrls.length)
    ]
  };

  tickets: Signal<TicketResponse[]> = signal([]);
  balanceHistory: Signal<BalanceHistory[]> = signal([]);

}
