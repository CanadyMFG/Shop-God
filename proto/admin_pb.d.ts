import * as jspb from "google-protobuf"

export class CreateUserRequest extends jspb.Message {
  getFirstname(): string;
  setFirstname(value: string): void;

  getLastname(): string;
  setLastname(value: string): void;

  getPin(): string;
  setPin(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateUserRequest): CreateUserRequest.AsObject;
  static serializeBinaryToWriter(message: CreateUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateUserRequest;
  static deserializeBinaryFromReader(message: CreateUserRequest, reader: jspb.BinaryReader): CreateUserRequest;
}

export namespace CreateUserRequest {
  export type AsObject = {
    firstname: string,
    lastname: string,
    pin: string,
  }
}

export class CreateUserReply extends jspb.Message {
  getRc(): number;
  setRc(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateUserReply.AsObject;
  static toObject(includeInstance: boolean, msg: CreateUserReply): CreateUserReply.AsObject;
  static serializeBinaryToWriter(message: CreateUserReply, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateUserReply;
  static deserializeBinaryFromReader(message: CreateUserReply, reader: jspb.BinaryReader): CreateUserReply;
}

export namespace CreateUserReply {
  export type AsObject = {
    rc: number,
  }
}

