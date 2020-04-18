class Squares {
    private readonly _n: number;

    constructor(n: number) {
        this._n = n;
    }

    get squareOfSum(): number {
        const sum = (this._n * (this._n+1)) / 2;
        return sum * sum;
    }
    
    get sumOfSquares(): number {
        const n = this._n;
        return n * (n+1) * (2*n + 1) / 6;
    }

    get difference(): number {
        return this.squareOfSum - this.sumOfSquares;
    }
}

export default Squares;