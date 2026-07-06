import { Component } from '@angular/core';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';
import { ContextHeader } from '../context-header/context-header';

@Component({
  selector: 'app-shell',
  imports: [RouterOutlet, RouterLink, RouterLinkActive, ContextHeader],
  templateUrl: './shell.component.html',
  styleUrl: './shell.component.scss',
})
export class Shell {}
