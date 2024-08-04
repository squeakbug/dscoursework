import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DialogFindErrorComponent } from './dialog-find-error.component';

describe('DialogFindErrorComponent', () => {
  let component: DialogFindErrorComponent;
  let fixture: ComponentFixture<DialogFindErrorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DialogFindErrorComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DialogFindErrorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
