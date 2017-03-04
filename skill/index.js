'use strict';
const Alexa = require('alexa-sdk');
const http = require('http');
const escape = require('escape-html');

exports.handler = function(event, context, callback) {
    const alexa = Alexa.handler(event, context);
    alexa.registerHandlers(handlers);
    alexa.execute();
};

const handlers = {
    'GetMenu': function () {
        const base_url = process.env.SERVERY_API_URL;
        if (base_url === undefined) {
            throw new Error('No SERVERY_API_URL specified');
        }

        let servery = this.event.request.intent.slots.Servery.value.toLowerCase();
        servery = serveryMap[servery] ? serveryMap[servery] : servery;
        
        http.get(`${base_url}/api/menu/${servery}/`, res => {
            const statusCode = res.statusCode;

            if (statusCode !== 200) {
                res.resume();
                throw new Error(`Menu request failed with status ${statusCode}`);
            }

            res.setEncoding('utf8');
            let rawData = '';
            res.on('data', chunk => rawData += chunk);
            res.on('end', () => {
                try {
                    const parsedData = JSON.parse(rawData);
                    const items = parsedData.payload;
                    const msg = escape(items.join(' '));
                    this.emit(':tell', msg);
                } catch (e) {
                    throw new Error('Could not parse menu response');
                }
            });
        });
    }
};

const serveryMap = {
    '4.taco': '4.Tac0',
    "sammy\'s": "@Sammy's",
    'Amb Cafe': 'Amb.Cafe',
    'baker': 'Baker',
    'coho': 'Coho',
    'foodtrucks': 'FoodTrucks',
    'north': 'North',
    'parlia-too': 'Parlia-Too',
    'parliament': 'Parliament',
    'salento': 'Salento',
    'seibel': 'Seibel',
    'sid rich': 'SidRich',
    'south': 'South',
    'west': 'West',
    'whoo deli': 'WhooDeli'
};