import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { map } from 'rxjs/operators';

import { environment } from '../../environments/environment';
import { User } from '../_model/user.model';

@Injectable({ providedIn: 'root' })
export class AdminService {
    constructor() {}//private http: HttpClient) {}

    addUser(user) {
        alert("Added User:" + user.firstName);
    }

    // getAllMembers() {
    //     return this.http.get<any>(`${environment.apiUrl}/member`).pipe(map((res) => {
    //         if(res.success) {
    //             return Object.values(res.members) as Member[];
    //         } else {
    //             return [];
    //         }
    //     }));
    // }

    // addMember(firstName: string, lastName: string, jobTitle: string, team: string, status: string) {
    //     return this.http.post(`${environment.apiUrl}/member/addmember`,{
    //         firstName,
    //         lastName,
    //         jobTitle,
    //         team,
    //         status
    //     });
    // }

}
