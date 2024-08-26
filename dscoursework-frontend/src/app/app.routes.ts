import { Route } from '@angular/router';
import { LoginComponent } from './auth/login/login.component';
import { RegisterComponent } from './auth/register/register.component';
import { IndexComponent } from './index/index-main/index.component';
import { UserProfileComponent } from './user/user-profile/user-profile.component';
import { NotFoundComponent } from './shared/not-found/not-found.component';

export const appRoutes: Route[] = [
  { path: '', redirectTo: '/index', pathMatch: 'full' },
  { path: 'index', component: IndexComponent },
  { path: 'login', component: LoginComponent },
  { path: 'register', component: RegisterComponent },
  { path: 'profile', component: UserProfileComponent },
  { path: '**', component: NotFoundComponent}
];
