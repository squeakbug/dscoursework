import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DialogFailedLoginComponent } from './dialog-failed-login.component';

describe('DialogFailedLoginComponent', () => {
  let component: DialogFailedLoginComponent;
  let fixture: ComponentFixture<DialogFailedLoginComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DialogFailedLoginComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DialogFailedLoginComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
