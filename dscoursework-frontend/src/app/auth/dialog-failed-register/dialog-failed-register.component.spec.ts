import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DialogFailedRegisterComponent } from './dialog-failed-register.component';

describe('DialogFailedRegisterComponent', () => {
  let component: DialogFailedRegisterComponent;
  let fixture: ComponentFixture<DialogFailedRegisterComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DialogFailedRegisterComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DialogFailedRegisterComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
