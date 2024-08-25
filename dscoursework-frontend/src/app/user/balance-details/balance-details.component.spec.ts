import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BalanceDetailsComponent } from './balance-details.component';

describe('BalanceDetailsComponent', () => {
  let component: BalanceDetailsComponent;
  let fixture: ComponentFixture<BalanceDetailsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BalanceDetailsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BalanceDetailsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

