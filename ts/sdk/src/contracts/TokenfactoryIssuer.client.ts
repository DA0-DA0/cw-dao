/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, AdditionalMetadata, DenomUnit, ExecuteMsg, Uint128, QueryMsg, SudoMsg, Coin, BlacklisteesResponse, StatusInfo, BlacklisterAllowancesResponse, AllowanceResponse, AllowancesResponse, AllowanceInfo, DenomResponse, FreezerAllowancesResponse, StatusResponse, IsFrozenResponse, OwnerResponse } from "./TokenfactoryIssuer.types";
export interface TokenfactoryIssuerReadOnlyInterface {
  contractAddress: string;
  isFrozen: () => Promise<IsFrozenResponse>;
  denom: () => Promise<DenomResponse>;
  owner: () => Promise<OwnerResponse>;
  burnAllowance: ({
    address
  }: {
    address: string;
  }) => Promise<AllowanceResponse>;
  burnAllowances: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<AllowancesResponse>;
  mintAllowance: ({
    address
  }: {
    address: string;
  }) => Promise<AllowanceResponse>;
  mintAllowances: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<AllowancesResponse>;
  isBlacklisted: ({
    address
  }: {
    address: string;
  }) => Promise<StatusResponse>;
  blacklistees: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<BlacklisteesResponse>;
  isBlacklister: ({
    address
  }: {
    address: string;
  }) => Promise<StatusResponse>;
  blacklisterAllowances: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<BlacklisterAllowancesResponse>;
  isFreezer: ({
    address
  }: {
    address: string;
  }) => Promise<StatusResponse>;
  freezerAllowances: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }) => Promise<FreezerAllowancesResponse>;
}
export class TokenfactoryIssuerQueryClient implements TokenfactoryIssuerReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.isFrozen = this.isFrozen.bind(this);
    this.denom = this.denom.bind(this);
    this.owner = this.owner.bind(this);
    this.burnAllowance = this.burnAllowance.bind(this);
    this.burnAllowances = this.burnAllowances.bind(this);
    this.mintAllowance = this.mintAllowance.bind(this);
    this.mintAllowances = this.mintAllowances.bind(this);
    this.isBlacklisted = this.isBlacklisted.bind(this);
    this.blacklistees = this.blacklistees.bind(this);
    this.isBlacklister = this.isBlacklister.bind(this);
    this.blacklisterAllowances = this.blacklisterAllowances.bind(this);
    this.isFreezer = this.isFreezer.bind(this);
    this.freezerAllowances = this.freezerAllowances.bind(this);
  }

  isFrozen = async (): Promise<IsFrozenResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      is_frozen: {}
    });
  };
  denom = async (): Promise<DenomResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      denom: {}
    });
  };
  owner = async (): Promise<OwnerResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      owner: {}
    });
  };
  burnAllowance = async ({
    address
  }: {
    address: string;
  }): Promise<AllowanceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      burn_allowance: {
        address
      }
    });
  };
  burnAllowances = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<AllowancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      burn_allowances: {
        limit,
        start_after: startAfter
      }
    });
  };
  mintAllowance = async ({
    address
  }: {
    address: string;
  }): Promise<AllowanceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      mint_allowance: {
        address
      }
    });
  };
  mintAllowances = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<AllowancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      mint_allowances: {
        limit,
        start_after: startAfter
      }
    });
  };
  isBlacklisted = async ({
    address
  }: {
    address: string;
  }): Promise<StatusResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      is_blacklisted: {
        address
      }
    });
  };
  blacklistees = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<BlacklisteesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      blacklistees: {
        limit,
        start_after: startAfter
      }
    });
  };
  isBlacklister = async ({
    address
  }: {
    address: string;
  }): Promise<StatusResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      is_blacklister: {
        address
      }
    });
  };
  blacklisterAllowances = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<BlacklisterAllowancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      blacklister_allowances: {
        limit,
        start_after: startAfter
      }
    });
  };
  isFreezer = async ({
    address
  }: {
    address: string;
  }): Promise<StatusResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      is_freezer: {
        address
      }
    });
  };
  freezerAllowances = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string;
  }): Promise<FreezerAllowancesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      freezer_allowances: {
        limit,
        start_after: startAfter
      }
    });
  };
}
export interface TokenfactoryIssuerInterface extends TokenfactoryIssuerReadOnlyInterface {
  contractAddress: string;
  sender: string;
  changeTokenFactoryAdmin: ({
    newAdmin
  }: {
    newAdmin: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  changeContractOwner: ({
    newOwner
  }: {
    newOwner: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setDenomMetadata: ({
    metadata
  }: {
    metadata: AdditionalMetadata;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setMinter: ({
    address,
    allowance
  }: {
    address: string;
    allowance: Uint128;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setBurner: ({
    address,
    allowance
  }: {
    address: string;
    allowance: Uint128;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setBlacklister: ({
    address,
    status
  }: {
    address: string;
    status: boolean;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setFreezer: ({
    address,
    status
  }: {
    address: string;
    status: boolean;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  mint: ({
    amount,
    toAddress
  }: {
    amount: Uint128;
    toAddress: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  burn: ({
    amount,
    fromAddress
  }: {
    amount: Uint128;
    fromAddress: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  blacklist: ({
    address,
    status
  }: {
    address: string;
    status: boolean;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  freeze: ({
    status
  }: {
    status: boolean;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class TokenfactoryIssuerClient extends TokenfactoryIssuerQueryClient implements TokenfactoryIssuerInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.changeTokenFactoryAdmin = this.changeTokenFactoryAdmin.bind(this);
    this.changeContractOwner = this.changeContractOwner.bind(this);
    this.setDenomMetadata = this.setDenomMetadata.bind(this);
    this.setMinter = this.setMinter.bind(this);
    this.setBurner = this.setBurner.bind(this);
    this.setBlacklister = this.setBlacklister.bind(this);
    this.setFreezer = this.setFreezer.bind(this);
    this.mint = this.mint.bind(this);
    this.burn = this.burn.bind(this);
    this.blacklist = this.blacklist.bind(this);
    this.freeze = this.freeze.bind(this);
  }

  changeTokenFactoryAdmin = async ({
    newAdmin
  }: {
    newAdmin: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      change_token_factory_admin: {
        new_admin: newAdmin
      }
    }, fee, memo, funds);
  };
  changeContractOwner = async ({
    newOwner
  }: {
    newOwner: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      change_contract_owner: {
        new_owner: newOwner
      }
    }, fee, memo, funds);
  };
  setDenomMetadata = async ({
    metadata
  }: {
    metadata: AdditionalMetadata;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_denom_metadata: {
        metadata
      }
    }, fee, memo, funds);
  };
  setMinter = async ({
    address,
    allowance
  }: {
    address: string;
    allowance: Uint128;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_minter: {
        address,
        allowance
      }
    }, fee, memo, funds);
  };
  setBurner = async ({
    address,
    allowance
  }: {
    address: string;
    allowance: Uint128;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_burner: {
        address,
        allowance
      }
    }, fee, memo, funds);
  };
  setBlacklister = async ({
    address,
    status
  }: {
    address: string;
    status: boolean;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_blacklister: {
        address,
        status
      }
    }, fee, memo, funds);
  };
  setFreezer = async ({
    address,
    status
  }: {
    address: string;
    status: boolean;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_freezer: {
        address,
        status
      }
    }, fee, memo, funds);
  };
  mint = async ({
    amount,
    toAddress
  }: {
    amount: Uint128;
    toAddress: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      mint: {
        amount,
        to_address: toAddress
      }
    }, fee, memo, funds);
  };
  burn = async ({
    amount,
    fromAddress
  }: {
    amount: Uint128;
    fromAddress: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      burn: {
        amount,
        from_address: fromAddress
      }
    }, fee, memo, funds);
  };
  blacklist = async ({
    address,
    status
  }: {
    address: string;
    status: boolean;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      blacklist: {
        address,
        status
      }
    }, fee, memo, funds);
  };
  freeze = async ({
    status
  }: {
    status: boolean;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      freeze: {
        status
      }
    }, fee, memo, funds);
  };
}