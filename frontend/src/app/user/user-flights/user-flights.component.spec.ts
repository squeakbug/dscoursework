import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UserFlightsComponent } from './user-flights.component';

describe('UserFlightsComponent', () => {
  let component: UserFlightsComponent;
  let fixture: ComponentFixture<UserFlightsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UserFlightsComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(UserFlightsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
