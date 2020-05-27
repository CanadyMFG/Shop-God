import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { AdduserComponent } from './adduser/adduser.component';
import { MustMatchDirective } from './_directives/mustmatch.directive';
import { AddCategoryComponent } from './add-category/add-category.component';
import { AddEntityComponent } from './add-entity/add-entity.component';

@NgModule({
  declarations: [
    AppComponent,
    MustMatchDirective,
    AdduserComponent,
    AddCategoryComponent,
    AddEntityComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    FormsModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
