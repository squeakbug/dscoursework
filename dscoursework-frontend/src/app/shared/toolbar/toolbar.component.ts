import { Component } from '@angular/core';
import { Router } from '@angular/router';
import { NgFor, NgIf } from '@angular/common';

import { MatToolbarModule } from '@angular/material/toolbar';
import { MatTabsModule, MatTabChangeEvent } from '@angular/material/tabs';
import { MatButtonModule } from '@angular/material/button';
import { AuthService } from 'src/app/services/auth.service';
import { map, Observable } from 'rxjs';

@Component({
  selector: 'app-toolbar',
  standalone: true,
  imports: [
    MatToolbarModule,
    MatTabsModule,
    MatButtonModule,
    NgFor, NgIf,
  ],
  templateUrl: './toolbar.component.html',
  styleUrl: './toolbar.component.scss'
})
export class ToolbarComponent {
  isAuthenticated$ = this.authService.isAuthenticated$.pipe(map(isAuth => {
    return isAuth.isAuthenticated;
  }));

  constructor(private router: Router, private authService: AuthService) {

  }

  login() {
    this.router.navigate(['/login']);
  }

  logout() {
    this.authService.logout();
  }

  selectedTabChangeHandle(e: MatTabChangeEvent) {
    switch (e.index) {
      case 0:
        this.router.navigate(['']);
        break;
      case 1:
        this.router.navigate(['/profile']);
        break;
      default:
        break;
    }
  }
}
