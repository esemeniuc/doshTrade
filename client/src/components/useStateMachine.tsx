import { useReducer } from "react";

export const buildMachineReducer = (spec: any) => (currentState: any, event: any) => {
    // We get all possible transitions for the current State
    const stateTransitions = spec.states[currentState];

    // No transitions? Error!
    if (stateTransitions === undefined) {
        throw new Error(`No transitions defined for ${currentState}`);
    }

    // We try to transition to the next state
    const nextState = stateTransitions[event];

    // No next state? Error!
    if (nextState === undefined) {
        throw new Error(
            `Unknown transition for event ${event} in state ${currentState}`
        );
    }

    // We return the new state
    return nextState;
};

export const useStateMachine = (spec: any) => {
    // Our hook is just a very thin wrapper around useReducer :)
    return useReducer(buildMachineReducer(spec), spec.initialState);
};