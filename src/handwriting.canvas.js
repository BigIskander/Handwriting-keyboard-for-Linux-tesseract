/* https://github.com/ChenYuHo/handwriting.js
The MIT License (MIT)

Copyright (c) 2015 Elton

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
(function(window, document) {
    // Establish the root object, `window` (`self`) in the browser, 
    // or `this` in some virtual machines. We use `self`
    // instead of `window` for `WebWorker` support.
    var root = typeof self === 'object' && self.self === self && self || this;

    // Create a safe reference to the handwriting object for use below.        
    var handwriting = function(obj) {
        if (obj instanceof handwriting) return obj;
        if (!(this instanceof handwriting)) return new handwriting(obj);
        this._wrapped = obj;
    };

    root.handwriting = handwriting;

    handwriting.Canvas = function(cvs, lineWidth) {
        this.canvas = cvs;
        this.cxt = cvs.getContext("2d");
        this.cxt.lineCap = "round";
        this.cxt.lineJoin = "round";
        this.lineWidth = lineWidth || 3;
        this.width = cvs.width;
        this.height = cvs.height;
        this.drawing = false;
        this.handwritingX = [];
        this.handwritingY = [];
        this.trace = [];
        this.options = {};
        this.step = [];
        this.redo_step = [];
        this.redo_trace = [];
        this.allowUndo = false;
        this.allowRedo = false;
        cvs.addEventListener("mousedown", this.mouseDown.bind(this));
        cvs.addEventListener("mousemove", this.mouseMove.bind(this));
        cvs.addEventListener("mouseup", this.mouseUp.bind(this));
        // slight modification, fixing some minor issues
        cvs.addEventListener("mouseout", this.mouseOut.bind(this)); 
        cvs.addEventListener("touchstart", this.touchStart.bind(this));
        cvs.addEventListener("touchmove", this.touchMove.bind(this));
        // ...
        cvs.addEventListener("touchend", this.touchEnd.bind(this));
        this.callback = undefined;
        this.mouseUpCallBack = undefined;
        this.recognize = handwriting.recognize;
        // to change stroke color for dark theme
        this.setStrokeColor = function(color) {
            this.cxt.strokeStyle = color;
        }
        this.setFillStyle = function(color) {
            this.cxt.fillStyle = color;
            this.cxt.fillRect(0, 0, this.canvas.width, this.canvas.height);
        }
    };
    /**
     * [toggle_Undo_Redo description]
     * @return {[type]} [description]
     */
    handwriting.Canvas.prototype.set_Undo_Redo = function(undo, redo) {
        this.allowUndo = undo;
        this.allowRedo = undo ? redo : false;
        if (!this.allowUndo) {
            this.step = [];
            this.redo_step = [];
            this.redo_trace = [];
        }
    };

    handwriting.Canvas.prototype.setLineWidth = function(lineWidth) {
        this.lineWidth = lineWidth;
    };

    handwriting.Canvas.prototype.setCallBack = function(callback) {
        this.callback = callback;
    };

    // slight modification to call parent module function
    handwriting.Canvas.prototype.setMouseUpCallBack = function(callback) {
        this.mouseUpCallBack = callback
    };

    handwriting.Canvas.prototype.setOptions = function(options) {
        this.options = options;
    };

    handwriting.Canvas.prototype.mouseDown = function(e) {
        // new stroke
        this.cxt.lineWidth = this.lineWidth;
        this.handwritingX = [];
        this.handwritingY = [];
        this.drawing = true;
        this.cxt.beginPath();
        var rect = this.canvas.getBoundingClientRect();
        var x = e.clientX - rect.left;
        var y = e.clientY - rect.top;
        this.cxt.moveTo(x, y);
        this.handwritingX.push(x);
        this.handwritingY.push(y);
    };


    handwriting.Canvas.prototype.mouseMove = function(e) {
        if (this.drawing) {
            var rect = this.canvas.getBoundingClientRect();
            var x = e.clientX - rect.left;
            var y = e.clientY - rect.top;
            this.cxt.lineTo(x, y);
            this.cxt.stroke();
            this.handwritingX.push(x);
            this.handwritingY.push(y);
        }
    };

    handwriting.Canvas.prototype.mouseUp = function() {
        var w = [];
        w.push(this.handwritingX);
        w.push(this.handwritingY);
        w.push([]);
        this.trace.push(w);
        this.drawing = false;
        if (this.allowUndo) this.step.push(this.canvas.toDataURL());
        this.mouseUpCallBack(); //slight modification to call parent module function
    };
    
    // slight modification, fixing some minor issues
    handwriting.Canvas.prototype.mouseOut = function() { 
        if(this.drawing) {
            var w = [];
            w.push(this.handwritingX);
            w.push(this.handwritingY);
            w.push([]);
            this.trace.push(w);
            this.drawing = false;
            if (this.allowUndo) this.step.push(this.canvas.toDataURL());
            this.mouseUpCallBack();
        }
    }

    handwriting.Canvas.prototype.touchStart = function(e) {
        e.preventDefault();
        this.cxt.lineWidth = this.lineWidth;
        this.handwritingX = [];
        this.handwritingY = [];
        var de = document.documentElement;
        var box = this.canvas.getBoundingClientRect();
        var top = box.top + window.scrollY - de.clientTop;
        var left = box.left + window.scrollX - de.clientLeft;
        var touch = e.changedTouches[0];
        var touchX = touch.pageX - left;
        var touchY = touch.pageY - top;
        this.handwritingX.push(touchX);
        this.handwritingY.push(touchY);
        this.cxt.beginPath();
        this.cxt.moveTo(touchX, touchY);
    };

    handwriting.Canvas.prototype.touchMove = function(e) {
        e.preventDefault();
        var touch = e.targetTouches[0];
        var de = document.documentElement;
        var box = this.canvas.getBoundingClientRect();
        var top = box.top + window.scrollY - de.clientTop;
        var left = box.left + window.scrollX - de.clientLeft;
        var x = touch.pageX - left;
        var y = touch.pageY - top;
        this.handwritingX.push(x);
        this.handwritingY.push(y);
        this.cxt.lineTo(x, y);
        this.cxt.stroke();
    };

    handwriting.Canvas.prototype.touchEnd = function(e) {
        var w = [];
        w.push(this.handwritingX);
        w.push(this.handwritingY);
        w.push([]);
        this.trace.push(w);
        if (this.allowUndo) this.step.push(this.canvas.toDataURL());
        this.mouseUpCallBack(); // slight modification to call parent module function
    };

    handwriting.Canvas.prototype.undo = function() {
        if (!this.allowUndo || this.step.length <= 0) return;
        else if (this.step.length === 1) {
            if (this.allowRedo) {
                this.redo_step.push(this.step.pop());
                this.redo_trace.push(this.trace.pop());
                this.cxt.clearRect(0, 0, this.width, this.height);
            }
        } else {
            if (this.allowRedo) {
                this.redo_step.push(this.step.pop());
                this.redo_trace.push(this.trace.pop());
            } else {
                this.step.pop();
                this.trace.pop();
            }
            loadFromUrl(this.step.slice(-1)[0], this);
        }
    };

    handwriting.Canvas.prototype.redo = function() {
        if (!this.allowRedo || this.redo_step.length <= 0) return;
        this.step.push(this.redo_step.pop());
        this.trace.push(this.redo_trace.pop());
        loadFromUrl(this.step.slice(-1)[0], this);
    };

    handwriting.Canvas.prototype.erase = function() {
        this.cxt.clearRect(0, 0, this.width, this.height);
        this.step = [];
        this.redo_step = [];
        this.redo_trace = [];
        this.trace = [];
    };

    function loadFromUrl(url, cvs) {
        var imageObj = new Image();
        imageObj.onload = function() {
            cvs.cxt.clearRect(0, 0, this.width, this.height);
            cvs.cxt.drawImage(imageObj, 0, 0);
        };
        imageObj.src = url;
    }

    // this function to call Google API is not used
    // tesseract-ocr API used instead
    /*
    handwriting.recognize = function(trace, options, callback) {
        if (handwriting.Canvas && this instanceof handwriting.Canvas) {
            trace = this.trace;
            options = this.options;
            callback = this.callback;
        } else if (!options) options = {};
        var data = JSON.stringify({
            "options": "enable_pre_space",
            "requests": [{
                "writing_guide": {
                    "writing_area_width": options.width || this.width || undefined,
                    "writing_area_height": options.height || this.width || undefined
                },
                "ink": trace,
                "language": options.language || "zh_TW"
            }]
        });
        var xhr = new XMLHttpRequest();
        xhr.addEventListener("readystatechange", function() {
            if (this.readyState === 4) {
                switch (this.status) {
                    case 0: 
                        //error handling no internet access or google not available
                        callback(undefined, new Error("ERR_NAME_NOT_RESOLVED"));
                        break;
                    case 200:
                        var response = JSON.parse(this.responseText);
                        var results;
                        if (response.length === 1) callback(undefined, new Error(response[0]));
                        else results = response[1][0][1];
                        if (!!options.numOfWords) {
                            results = results.filter(function(result) {
                                return (result.length == options.numOfWords);
                            });
                        }
                        if (!!options.numOfReturn) {
                            results = results.slice(0, options.numOfReturn);
                        }
                        callback(results, undefined);
                        break;
                    case 403:
                        callback(undefined, new Error("access denied"));
                        break;
                    case 503:
                        callback(undefined, new Error("can't connect to recognition server"));
                        break;
                }


            }
        });
        xhr.open("POST", "https://www.google.com/inputtools/request?ime=handwriting&app=mobilesearch&cs=1&oe=UTF-8");
        xhr.setRequestHeader("content-type", "application/json");
        xhr.send(data);
    };*/

})(window, document);