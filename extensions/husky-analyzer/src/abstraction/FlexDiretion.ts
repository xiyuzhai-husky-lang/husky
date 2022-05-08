export default class FlexDirection {
    is_row: boolean;
    constructor(is_row: boolean) {
        this.is_row = is_row;
    }

    transpose(): FlexDirection {
        return new FlexDirection(!this.is_row);
    }

    code(): string {
        if (this.is_row) {
            return "row";
        } else {
            return "column";
        }
    }
}

export const FLEX_DIRECTION_ROW = new FlexDirection(true);
export const FLEX_DIRECTION_COLUMN = new FlexDirection(false);
