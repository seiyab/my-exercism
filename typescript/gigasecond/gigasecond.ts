const giga = Math.pow(10, 9);

class GigaSecond {
    private readonly _date: Date;

    constructor(date: Date) {
        const gigaTime = date.getTime() + giga * 1000;
        this._date = new Date(gigaTime);
    }

    date(): Date {
        return new Date(this._date)
    }
}

export default GigaSecond;