
export type ActionMap<M extends { [index: string]: any }> = {
    [Key in keyof M]: M[Key] extends undefined
      ? {
          type: Key;
        }
      : {
          type: Key;
          payload: M[Key];
        }
  };


export enum PushActionTypes {
    USER_PERMISSION = 'USER_PERMISSION',
}  

export type PushStateType = {
    userConsent: NotificationPermission
}

export type PushPayload = {
    [PushActionTypes.USER_PERMISSION] : {
      userConsent: NotificationPermission;
    };
}

export type PushActions = ActionMap<PushPayload>[keyof ActionMap<PushPayload>];
