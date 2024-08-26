import { Component, Input, OnInit } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';

import { TicketResponse } from 'src/app/models/TicketResponse';
import { sampleAnimeHeroesImageUrls } from 'src/assets/sample.animeHeroesImagesUrls';
import { TicketRepository } from 'src/app/services/ticket.repository';

@Component({
  standalone: true,
  selector: 'app-ticket-details',
  templateUrl: './ticket-details.component.html',
  styleUrls: ['./ticket-details.component.scss'],
  imports: [ 
    MatButtonModule, MatCardModule 
  ],
})
export class TicketDetailsComponent implements OnInit {
  image: String = sampleAnimeHeroesImageUrls[
    Math.floor(Math.random() * sampleAnimeHeroesImageUrls.length)
  ];
  @Input() ticket: TicketResponse | null = null;

  constructor(
    private ticketRepository: TicketRepository
  ) { }

  ngOnInit(): void {

  }

  cancelOrder() {
    this.ticketRepository.deleteTicket(this.ticket?.ticketUid!);
  }
}
