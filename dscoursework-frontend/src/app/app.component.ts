import { Component } from '@angular/core';
import { RouterModule, RouterOutlet } from '@angular/router';

import { ToolbarComponent } from 'src/app/shared/toolbar/toolbar.component';
import { FooterComponent } from 'src/app/shared/footer/footer.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    RouterOutlet,
    RouterModule,
    ToolbarComponent,
    FooterComponent,
  ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  title = 'FlightBookingFrontend';
}
