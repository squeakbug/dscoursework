import { Component } from '@angular/core';

import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatFormFieldModule } from '@angular/material/form-field';
import { provideNativeDateAdapter } from '@angular/material/core';

import { ToolbarComponent } from 'src/app/shared/toolbar/toolbar.component';
import { FooterComponent } from 'src/app/shared/footer/footer.component';

@Component({
  selector: 'app-index',
  standalone: true,
  imports: [
    MatInputModule,
    MatButtonModule,
    MatDatepickerModule,
    MatFormFieldModule,
    ToolbarComponent,
    FooterComponent,
  ],
  templateUrl: './index.component.html',
  styleUrl: './index.component.scss',
  providers: [provideNativeDateAdapter()],
})
export class IndexComponent {

}
