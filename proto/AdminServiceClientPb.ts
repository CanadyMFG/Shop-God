/**
 * @fileoverview gRPC-Web generated client stub for admin_actions
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


import * as grpcWeb from 'grpc-web';

import {
  CreateUserReply,
  CreateUserRequest} from './admin_pb';

export class UserManagerClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: string; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: string; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'text';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname;
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodInfoCreateUser = new grpcWeb.AbstractClientBase.MethodInfo(
    CreateUserReply,
    (request: CreateUserRequest) => {
      return request.serializeBinary();
    },
    CreateUserReply.deserializeBinary
  );

  createUser(
    request: CreateUserRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: CreateUserReply) => void) {
    return this.client_.rpcCall(
      this.hostname_ +
        '/admin_actions.UserManager/CreateUser',
      request,
      metadata || {},
      this.methodInfoCreateUser,
      callback);
  }

}

