export default class Dict<T> {
    data: { [key: string]: T } = {};

    clear() {
        this.data = {};
    }

    insert_new(key: string, value: T) {
        console.assert(!(key in this.data));
        this.data[key] = value;
    }

    get(key: string, gen_error_message: () => string) {
        console.assert(key in this.data, gen_error_message());
        return this.data[key];
    }

    has(key: string): boolean {
        return key in this.data;
    }
}
