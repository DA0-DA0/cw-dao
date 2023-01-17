/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {
  owner?: string | null;
  params: UncheckedVestingParams;
}
export type ExecuteMsg = {
  receive: Cw20ReceiveMsg;
} | {
  instantiate_native_payroll_contract: {
    instantiate_msg: InstantiateMsg;
    label: string;
  };
} | {
  update_code_id: {
    vesting_code_id: number;
  };
} | {
  update_ownership: Action;
};
export type Uint128 = string;
export type Binary = string;
export type UncheckedDenom = {
  native: string;
} | {
  cw20: string;
};
export type Curve = {
  constant: {
    y: Uint128;
    [k: string]: unknown;
  };
} | {
  saturating_linear: SaturatingLinear;
} | {
  piecewise_linear: PiecewiseLinear;
};
export type Action = {
  transfer_ownership: {
    expiry?: Expiration | null;
    new_owner: string;
  };
} | "accept_ownership" | "renounce_ownership";
export type Expiration = {
  at_height: number;
} | {
  at_time: Timestamp;
} | {
  never: {};
};
export type Timestamp = Uint64;
export type Uint64 = string;
export interface Cw20ReceiveMsg {
  amount: Uint128;
  msg: Binary;
  sender: string;
}
export interface UncheckedVestingParams {
  amount: Uint128;
  denom: UncheckedDenom;
  description?: string | null;
  recipient: string;
  title?: string | null;
  vesting_schedule: Curve;
}
export interface SaturatingLinear {
  max_x: number;
  max_y: Uint128;
  min_x: number;
  min_y: Uint128;
  [k: string]: unknown;
}
export interface PiecewiseLinear {
  steps: [number, Uint128][];
  [k: string]: unknown;
}
export type QueryMsg = {
  list_vesting_contracts: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  list_vesting_contracts_reverse: {
    limit?: number | null;
    start_before?: string | null;
  };
} | {
  list_vesting_contracts_by_instantiator: {
    instantiator: string;
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  list_vesting_contracts_by_instantiator_reverse: {
    instantiator: string;
    limit?: number | null;
    start_before?: string | null;
  };
} | {
  list_vesting_contracts_by_recipient: {
    limit?: number | null;
    recipient: string;
    start_after?: string | null;
  };
} | {
  list_vesting_contracts_by_recipient_reverse: {
    limit?: number | null;
    recipient: string;
    start_before?: string | null;
  };
} | {
  ownership: {};
} | {
  code_id: {};
};
export type Addr = string;
export type ArrayOfAddr = Addr[];
export interface OwnershipForAddr {
  owner?: Addr | null;
  pending_expiry?: Expiration | null;
  pending_owner?: Addr | null;
}