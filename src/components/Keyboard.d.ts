export enum EButtonType { Digit = 'digit', Clear = 'clear', Enter = 'enter' };

export interface IButton {
  type: EButtonType;
  text: string;
  value: number | string;
}