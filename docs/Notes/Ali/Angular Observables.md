# Angular Observable Subscribe 

## Observable

A sequence of data emited asynchronously (most of the times) over period of time. 

## Observer

An observer continously listens to an observer to keep track of data changes. 

## Subscribe

The subscribe method listens to the data / error sent by the observable. 

## toPromise Implementation

Used when data doesn't being emited does not change. Only listen once. Stop listening for more data changes i.e. activate user. 



## Demo Notes

There are two ways to create observables - 

1. Using RxJS (using the of operator)
2. Using the new 'Observable' keyword (using an observer and subscribe)

Note: Always add $ to variables to identify it as an observable

