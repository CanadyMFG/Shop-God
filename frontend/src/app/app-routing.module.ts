import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { AdduserComponent } from './adduser/adduser.component';
import { AddCategoryComponent } from './add-category/add-category.component';
import { AddEntityComponent } from './add-entity/add-entity.component';


const routes: Routes = [
  {path: "adduser", component: AdduserComponent},
  {path: "addcategory", component: AddCategoryComponent},
  {path: "addentity", component: AddEntityComponent},
  {path:  "**", pathMatch:  "full", redirectTo:  "adduser"}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
