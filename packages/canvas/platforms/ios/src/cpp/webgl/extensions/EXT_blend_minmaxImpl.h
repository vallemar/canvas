//
// Created by Osei Fortune on 28/04/2022.
//

#pragma once

#include "Common.h"
#include <vector>
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
class EXT_blend_minmaxImpl: ObjectWrapperImpl {
public:
    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, EXT_blend_minmaxImpl *minmax) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = EXT_blend_minmaxImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType( object, NativeType::EXT_blend_minmax);
        object->SetAlignedPointerInInternalField(0, minmax);
        object->Set(context, ConvertToV8String(isolate, "ext_name"),
                    ConvertToV8String(isolate, "EXT_blend_minmax"));
        minmax->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static EXT_blend_minmaxImpl *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetAlignedPointerFromInternalField(0);
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<EXT_blend_minmaxImpl *>(ptr);
    }
};
