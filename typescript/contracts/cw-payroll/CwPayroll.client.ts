/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, Uint128, Binary, Cw20ReceiveMsg, QueryMsg, ConfigResponse, WrappedBalance, NativeBalance, Addr, WrappedBalance1, StreamResponse, Coin, Cw20CoinVerified, ListStreamsResponse } from "./CwPayroll.types";
export interface CwPayrollReadOnlyInterface {
  contractAddress: string;
  getConfig: () => Promise<ConfigResponse>;
  getStream: ({
    id
  }: {
    id: number;
  }) => Promise<StreamResponse>;
  listStreams: ({
    limit,
    start
  }: {
    limit?: number;
    start?: number;
  }) => Promise<ListStreamsResponse>;
}
export class CwPayrollQueryClient implements CwPayrollReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getConfig = this.getConfig.bind(this);
    this.getStream = this.getStream.bind(this);
    this.listStreams = this.listStreams.bind(this);
  }

  getConfig = async (): Promise<ConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_config: {}
    });
  };
  getStream = async ({
    id
  }: {
    id: number;
  }): Promise<StreamResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_stream: {
        id
      }
    });
  };
  listStreams = async ({
    limit,
    start
  }: {
    limit?: number;
    start?: number;
  }): Promise<ListStreamsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      list_streams: {
        limit,
        start
      }
    });
  };
}
export interface CwPayrollInterface extends CwPayrollReadOnlyInterface {
  contractAddress: string;
  sender: string;
  receive: ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  distribute: ({
    id
  }: {
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  pauseStream: ({
    id
  }: {
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  linkStream: ({
    leftStreamId,
    rightStreamId
  }: {
    leftStreamId: number;
    rightStreamId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  detachStream: ({
    leftStreamId,
    rightStreamId
  }: {
    leftStreamId: number;
    rightStreamId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  resumeStream: ({
    id
  }: {
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeStream: ({
    id
  }: {
    id: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CwPayrollClient extends CwPayrollQueryClient implements CwPayrollInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.receive = this.receive.bind(this);
    this.distribute = this.distribute.bind(this);
    this.pauseStream = this.pauseStream.bind(this);
    this.linkStream = this.linkStream.bind(this);
    this.detachStream = this.detachStream.bind(this);
    this.resumeStream = this.resumeStream.bind(this);
    this.removeStream = this.removeStream.bind(this);
  }

  receive = async ({
    amount,
    msg,
    sender
  }: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      receive: {
        amount,
        msg,
        sender
      }
    }, fee, memo, funds);
  };
  distribute = async ({
    id
  }: {
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      distribute: {
        id
      }
    }, fee, memo, funds);
  };
  pauseStream = async ({
    id
  }: {
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      pause_stream: {
        id
      }
    }, fee, memo, funds);
  };
  linkStream = async ({
    leftStreamId,
    rightStreamId
  }: {
    leftStreamId: number;
    rightStreamId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      link_stream: {
        left_stream_id: leftStreamId,
        right_stream_id: rightStreamId
      }
    }, fee, memo, funds);
  };
  detachStream = async ({
    leftStreamId,
    rightStreamId
  }: {
    leftStreamId: number;
    rightStreamId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      detach_stream: {
        left_stream_id: leftStreamId,
        right_stream_id: rightStreamId
      }
    }, fee, memo, funds);
  };
  resumeStream = async ({
    id
  }: {
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      resume_stream: {
        id
      }
    }, fee, memo, funds);
  };
  removeStream = async ({
    id
  }: {
    id: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_stream: {
        id
      }
    }, fee, memo, funds);
  };
}