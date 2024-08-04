import { Component, Input, OnInit } from '@angular/core';

import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatDialog } from '@angular/material/dialog';
import { MatDialogModule } from '@angular/material/dialog';

import { FlightResponse } from 'src/app/services';
import { DialogFlightBuyComponent } from '../dialog-flight-buy/dialog-flight-buy.component';
import { sampleAnimeHeroesImageUrls } from 'src/assets/sample.animeHeroesImagesUrls';

@Component({
  standalone: true,
  selector: 'flight-detail',
  templateUrl: './flight-detail.component.html',
  styleUrls: ['./flight-detail.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatDialogModule,
  ],
})
export class FlightDetailComponent implements OnInit {
  image: String = sampleAnimeHeroesImageUrls[Math.floor(Math.random() * sampleAnimeHeroesImageUrls.length)];
  @Input() flight: FlightResponse | null = null;

  constructor(public dialog: MatDialog) {

  }

  ngOnInit(): void {

  }

  openDialog(enterAnimationDuration: string, exitAnimationDuration: string): void {
    let dialogRef = this.dialog.open(DialogFlightBuyComponent, {
      enterAnimationDuration,
      exitAnimationDuration,
    });
    let instance = dialogRef.componentInstance;
    instance.flight = this.flight;
  }
}
