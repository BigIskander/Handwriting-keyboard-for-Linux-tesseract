import sys
from paddleocr import PaddleOCR
# to sort the results
from functools import cmp_to_key

img_bytes = sys.stdin.buffer.read()
lang = sys.argv[1]
det = False
if len(sys.argv) > 2:
    det = (sys.argv[2] == "multiline")

# is positions or text results one line or not
def is_paddleocr_result_positions_inline(bbox1, bbox2):
    return not (bbox1[0][1][1] > bbox2[0][3][1] or bbox1[0][3][1] < bbox2[0][1][1])

# compare paddle ocr results to sort them
def compare_paddleocr_result_positions(bbox1, bbox2):
    if is_paddleocr_result_positions_inline(bbox1, bbox2):
        return bbox1[0][0][0] - bbox2[0][0][0]
    else:
        return bbox1[0][0][1] - bbox2[0][0][1]

ocr = PaddleOCR(use_angle_cls=True, lang=lang) # need to run only once to load model into memory
result = ocr.ocr(img_bytes, det=det, cls=True) # det=False, 
for idx in range(len(result)):
    res = result[idx]
    if res:
        if det:
            output = ""
            res = sorted(res, key=cmp_to_key(compare_paddleocr_result_positions))
            res = sorted(res, key=cmp_to_key(compare_paddleocr_result_positions))
            prev = None
            # make somewhat simmilar output, so same regex can be used
            for line in res:    
                if prev != None:
                    if is_paddleocr_result_positions_inline(line, prev):
                        output = output + "\t" + line[1][0]
                    else:
                        print(" ppocr INFO: ('" + output + "', " + str(prev[1][1]) + ") ")
                        output = line[1][0]
                else:
                    output = line[1][0]
                prev = line
            if output != "" and prev != None:
                print(" ppocr INFO: ('" + output + "', " + str(prev[1][1]) + ") ")
        else:
            for line in res:
                print(" ppocr INFO: ('" + line[0] + "', " + str(line[1]) + ") ")