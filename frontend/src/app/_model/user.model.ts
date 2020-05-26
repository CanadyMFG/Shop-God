export class User {
    firstName: string;
    lastName: string;
    password: string;

    constructor(user) {
        this.firstName = user.firstName;
        this.lastName = user.lastName;
        this.password = user.password;
    }
}