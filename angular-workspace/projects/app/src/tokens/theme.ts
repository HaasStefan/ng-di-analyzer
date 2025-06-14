import { Component, InjectionToken } from "@angular/core";

export const THEME = new InjectionToken<'light' | 'dark'>("theme");



// This is not used in the app, and will be tree-shaken out, and should therefore not be included in the AngularProgram
@Component({
  selector: "app-theme",
  standalone: true,
  providers: [
    {
      provide: THEME,
      useValue: 'light' // Default theme can be set here
    }
  ],
  template: `
    <ng-content></ng-content>
  `
})
export class ThemeComponent {
  constructor() {
    // You can add any initialization logic here if needed
  }
}