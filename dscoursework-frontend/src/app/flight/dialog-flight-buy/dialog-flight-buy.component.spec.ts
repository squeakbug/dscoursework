import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DialogFlightBuyComponent } from './dialog-flight-buy.component';

describe('FlightDetailComponent', () => {
  let component: DialogFlightBuyComponent;
  let fixture: ComponentFixture<DialogFlightBuyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DialogFlightBuyComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DialogFlightBuyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
