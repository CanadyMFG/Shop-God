export class Entity {
    _id: string;
    name: string;
    quantity: number;
    alertNum: number;
    manufacturer: string;
    manufacturerNum: string;
    category: string;

    constructor(entity) {
        this.name = entity.name;
        this.quantity = entity.quantity;
        this.alertNum = entity.alertNum;
        this.manufacturer = entity.manufacturer;
        this.manufacturerNum = entity.manufacturerNum;
        this.category = entity.category;
    }
}