import { inject } from "@angular/core";
import { 
  ActivatedRouteSnapshot, CanActivateChildFn, 
  CanActivateFn, Router, RouterStateSnapshot 
} from "@angular/router";

import { AuthService } from "./auth.service";
import { map, Observable, take, tap } from "rxjs";

export const canActivateAuthentificated: CanActivateFn = (
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot,
  ): Observable<boolean> => {
  const authService = inject(AuthService);
  const router = inject(Router)

  return authService.isAuthenticated$.pipe(
    map((x) => {
      if (x.isAuthenticated) {
        return true
      } else {
        router.navigate(['/login'])
        return false
      }
    })
  )
};

export const canActivateNotAuthentificated: CanActivateFn = (
  route: ActivatedRouteSnapshot,
  state: RouterStateSnapshot,
): Observable<boolean> => {
  const authService = inject(AuthService);
  const router = inject(Router)

  return authService.isAuthenticated$.pipe(
    map((x) => {
      if (!x.isAuthenticated) {
        return true
      } else {
        router.navigate(['/index'])
        return false
      }
    })
  )
};
  
export const canActivateAuthentificatedChild: CanActivateChildFn = (
  route: ActivatedRouteSnapshot, 
  state: RouterStateSnapshot
) => canActivateAuthentificated(route, state);

export const canActivateNotAuthentificatedChild: CanActivateChildFn = (
  route: ActivatedRouteSnapshot, 
  state: RouterStateSnapshot
) => canActivateNotAuthentificated(route, state);
