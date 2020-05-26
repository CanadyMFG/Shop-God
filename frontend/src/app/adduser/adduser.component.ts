import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators, NgForm } from '@angular/forms';
import { Router } from '@angular/router';

import { AdminService } from '../_service/admin.service';
import { User } from '../_model/user.model';

@Component({
  selector: 'app-adduser',
  templateUrl: './adduser.component.html',
  styleUrls: ['./adduser.component.css']
})
export class AdduserComponent implements OnInit {

  model: any = {};
  constructor(private router: Router, private adminService: AdminService) { }

  ngOnInit(): void {
    
  }

  onSubmit() {
    let user = new User(this.model);
    this.adminService.addUser(user)
  }

}
