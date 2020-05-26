import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators, NgForm } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-adduser',
  templateUrl: './adduser.component.html',
  styleUrls: ['./adduser.component.css']
})
export class AdduserComponent implements OnInit {

  model: any = {};
  constructor(private router: Router) { }

  ngOnInit(): void {
    
  }

  onSubmit() {
    alert('SUCCESS!\n\n' + JSON.stringify(this.model, null, 4));
  }

}
