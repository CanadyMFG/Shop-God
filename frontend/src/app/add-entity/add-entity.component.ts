import { Component, OnInit } from '@angular/core';
import { Entity } from '../_model/entity.model';

@Component({
  selector: 'app-add-entity',
  templateUrl: './add-entity.component.html',
  styleUrls: ['./add-entity.component.css']
})
export class AddEntityComponent implements OnInit {
  model: any = {};
  constructor() { }

  ngOnInit(): void {
  }

  onSubmit() {
    let entity = new Entity(this.model);
    alert(JSON.stringify(entity));
  }

}
