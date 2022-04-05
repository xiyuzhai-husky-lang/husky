export default class NDict<T> {
    data: { [key: number]: T } = {};

    clear() {
        this.data = {};
    }

    insert_new(key: number, value: T) {
        console.assert(!(key in this.data));
        this.data[key] = value;
    }

    get(key: number) {
        console.assert(key in this.data);
        return this.data[key];
    }
}
